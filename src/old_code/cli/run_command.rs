use crate::app::cli::command::{CLI, CLICommand, GenerateCommand, SeedCommandAction};
use crate::app::app_ctx::AppCtx;
use crate::migrate::migrate;
use crate::core::result::Result;
use crate::core::conf::test::{ResetDatasets, ResetMode};
use crate::server::test_context::{TestContext};
use crate::purger::purge;
use crate::seeder::seed::seed;
use crate::server::serve;
use crate::gen::interface::server::gen::gen as gen_entity;
use crate::gen::interface::client::gen::gen as gen_client;
use crate::prelude::UserCtx;
use crate::run::run_program;

pub(crate) async fn run_command(cli: &CLI) -> Result<()> {
    let app_ctx = AppCtx::get()?;
    let graph = app_ctx.graph();
    let datasets = app_ctx.datasets();
    match &cli.command {
        CLICommand::Serve(serve_command) => {
            if !serve_command.no_migration {
                migrate(app_ctx.graph(), false).await?;
            }
            let env = serve_command.env.as_ref().cloned().unwrap_or(std::env::var("TEO_ENV").unwrap_or("debug".to_string()));
            let test_context: Option<&'static TestContext> = if env.as_str() == "test" {
                if let Some(test_conf) = app_ctx.test_conf() {
                    if let Some(reset) = &test_conf.reset {
                        match &reset.datasets {
                            ResetDatasets::Auto => Some(Box::leak(Box::new(TestContext {
                                reset_mode: ResetMode::AfterQuery,
                                datasets: app_ctx.datasets().iter().filter(|d| d.autoseed == true).map(|d| (*d).clone()).collect(),
                            }))),
                            ResetDatasets::Names(names) => {
                                let sv: Vec<Vec<String>> = names.iter().map(|v| v.to_string().split(".").map(|s| s.to_string()).collect()).collect();
                                Some(Box::leak(Box::new(TestContext {
                                    reset_mode: ResetMode::AfterQuery,
                                    datasets: app_ctx.datasets().iter().filter(|d| sv.contains(&d.name)).map(|d| (*d).clone()).collect(),
                                })))
                            }
                        }
                    } else {
                        None
                    }
                } else { None }
            } else { None };
            app_ctx.set_test_context(test_context)?;
            if let Some(test_context) = test_context {
                app_ctx.connector()?.connection().await?.purge().await.unwrap();
                seed(SeedCommandAction::Seed, graph, test_context.datasets.iter().collect(), test_context.datasets.iter().map(|d| d.name.join(".")).collect()).await?;
            } else if !serve_command.no_autoseed && !datasets.is_empty() {
                let names: Vec<String> = datasets.iter().filter_map(|d| if d.autoseed { Some(d.name.join(".")) } else { None }).collect();
                if !names.is_empty() {
                    seed(SeedCommandAction::Seed, graph, datasets, names).await?;
                }
            }
            if let Some(setup_callback) = app_ctx.setup() {
                let user_ctx = UserCtx::new(app_ctx.connector()?.connection().await?, None);
                setup_callback.call(user_ctx).await.unwrap();
            }
            serve(
                graph,
                app_ctx.server_conf()?,
                app_ctx.langauge_platform(),
                app_ctx.entrance(),
                app_ctx.middlewares(),
            ).await?
        }
    }
    Ok(())
}