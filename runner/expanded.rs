#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use clap::{Args, Parser, Subcommand, ValueEnum};
fn main() {
    let args = Cli::parse();
    {
        ::std::io::_print(
            ::core::fmt::Arguments::new_v1(
                &["", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&args)],
            ),
        );
    };
}
struct Cli {
    arch: Arch,
    #[command(subcommand)]
    command: Commands,
}
impl clap::Parser for Cli {}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::CommandFactory for Cli {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("runner");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("runner");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::FromArgMatches for Cli {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = Cli {
            arch: __clap_arg_matches
                .remove_one::<Arch>("arch")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["The following required argument was not provided: "],
                                &[::core::fmt::ArgumentV1::new_display(&"arch")],
                            ),
                        );
                        res
                    },
                ))?,
            command: {
                <Commands as clap::FromArgMatches>::from_arg_matches_mut(
                    __clap_arg_matches,
                )?
            },
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("arch") {
            #[allow(non_snake_case)]
            let arch = &mut self.arch;
            *arch = __clap_arg_matches
                .remove_one::<Arch>("arch")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["The following required argument was not provided: "],
                                &[::core::fmt::ArgumentV1::new_display(&"arch")],
                            ),
                        );
                        res
                    },
                ))?;
        }
        {
            #[allow(non_snake_case)]
            let command = &mut self.command;
            <Commands as clap::FromArgMatches>::update_from_arg_matches_mut(
                command,
                __clap_arg_matches,
            )?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::Args for Cli {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("Cli"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Cli")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("arch")];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("arch")
                        .value_name("ARCH")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<Arch>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg;
                    let arg = arg;
                    arg
                });
            let __clap_app = <Commands as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true);
            __clap_app
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Cli")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("arch")];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("arch")
                        .value_name("ARCH")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<Arch>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg;
                    let arg = arg.required(false);
                    arg
                });
            let __clap_app = <Commands as clap::Subcommand>::augment_subcommands(
                __clap_app,
            );
            let __clap_app = __clap_app
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand_required(false)
                .arg_required_else_help(false);
            __clap_app
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Cli {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Cli",
            "arch",
            &&self.arch,
            "command",
            &&self.command,
        )
    }
}
enum Commands {
    Run(RunArgs),
    Build(BuildArgs),
    ObjDump(ObjDumpArgs),
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::FromArgMatches for Commands {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches))
            = __clap_arg_matches.remove_subcommand()
        {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "run" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(
                    Self::Run(
                        <RunArgs as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ),
                );
            }
            if __clap_name == "build" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(
                    Self::Build(
                        <BuildArgs as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ),
                );
            }
            if __clap_name == "obj-dump" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(
                    Self::ObjDump(
                        <ObjDumpArgs as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?,
                    ),
                );
            }
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::InvalidSubcommand,
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["The subcommand \'", "\' wasn\'t recognized"],
                                &[::core::fmt::ArgumentV1::new_display(&__clap_name)],
                            ),
                        );
                        res
                    },
                ),
            )
        } else {
            ::std::result::Result::Err(
                clap::Error::raw(
                    clap::error::ErrorKind::MissingSubcommand,
                    "A subcommand is required but one was not provided.",
                ),
            )
        }
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::Run(ref mut __clap_arg) if "run" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    clap::FromArgMatches::update_from_arg_matches_mut(
                        __clap_arg,
                        __clap_arg_matches,
                    )?
                }
                Self::Build(ref mut __clap_arg) if "build" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    clap::FromArgMatches::update_from_arg_matches_mut(
                        __clap_arg,
                        __clap_arg_matches,
                    )?
                }
                Self::ObjDump(ref mut __clap_arg) if "obj-dump" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                        .remove_subcommand()
                        .unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    clap::FromArgMatches::update_from_arg_matches_mut(
                        __clap_arg,
                        __clap_arg_matches,
                    )?
                }
                s => {
                    *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?;
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::Subcommand for Commands {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("run");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <RunArgs as clap::Args>::augment_args(__clap_subcommand)
                };
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("build");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <BuildArgs as clap::Args>::augment_args(__clap_subcommand)
                };
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("obj-dump");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <ObjDumpArgs as clap::Args>::augment_args(__clap_subcommand)
                };
                __clap_subcommand
            });
        __clap_app
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("run");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <RunArgs as clap::Args>::augment_args_for_update(__clap_subcommand)
                };
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("build");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <BuildArgs as clap::Args>::augment_args_for_update(__clap_subcommand)
                };
                __clap_subcommand
            });
        let __clap_app = __clap_app
            .subcommand({
                let __clap_subcommand = clap::Command::new("obj-dump");
                let __clap_subcommand = __clap_subcommand;
                let __clap_subcommand = {
                    <ObjDumpArgs as clap::Args>::augment_args_for_update(
                        __clap_subcommand,
                    )
                };
                __clap_subcommand
            });
        __clap_app
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        if "run" == __clap_name {
            return true;
        }
        if "build" == __clap_name {
            return true;
        }
        if "obj-dump" == __clap_name {
            return true;
        }
        false
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Commands {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Commands::Run(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Run", &__self_0)
            }
            Commands::Build(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Build", &__self_0)
            }
            Commands::ObjDump(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "ObjDump",
                    &__self_0,
                )
            }
        }
    }
}
struct RunArgs {
    pub emulator: Emulator,
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::FromArgMatches for RunArgs {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = RunArgs {
            emulator: __clap_arg_matches
                .remove_one::<Emulator>("emulator")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["The following required argument was not provided: "],
                                &[::core::fmt::ArgumentV1::new_display(&"emulator")],
                            ),
                        );
                        res
                    },
                ))?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("emulator") {
            #[allow(non_snake_case)]
            let emulator = &mut self.emulator;
            *emulator = __clap_arg_matches
                .remove_one::<Emulator>("emulator")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["The following required argument was not provided: "],
                                &[::core::fmt::ArgumentV1::new_display(&"emulator")],
                            ),
                        );
                        res
                    },
                ))?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::Args for RunArgs {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("RunArgs"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("RunArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [
                                clap::Id::from("emulator"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("emulator")
                        .value_name("EMULATOR")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<
                                Emulator,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg;
                    let arg = arg;
                    arg
                });
            __clap_app
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("RunArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [
                                clap::Id::from("emulator"),
                            ];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("emulator")
                        .value_name("EMULATOR")
                        .required(true && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap::builder::via_prelude::*;
                            let auto = ::clap::builder::_AutoValueParser::<
                                Emulator,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg;
                    let arg = arg.required(false);
                    arg
                });
            __clap_app
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for RunArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "RunArgs",
            "emulator",
            &&self.emulator,
        )
    }
}
struct BuildArgs {}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::FromArgMatches for BuildArgs {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = BuildArgs {};
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::Args for BuildArgs {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("BuildArgs"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("BuildArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            __clap_app
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("BuildArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            __clap_app
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for BuildArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "BuildArgs")
    }
}
struct ObjDumpArgs {}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::FromArgMatches for ObjDumpArgs {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = ObjDumpArgs {};
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::Args for ObjDumpArgs {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("ObjDumpArgs"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("ObjDumpArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            __clap_app
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("ObjDumpArgs")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 0usize] = [];
                            members
                        }),
                );
            __clap_app
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for ObjDumpArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "ObjDumpArgs")
    }
}
pub enum Arch {
    X86_64,
    Aarch64,
}
#[automatically_derived]
impl ::core::marker::Copy for Arch {}
#[automatically_derived]
impl ::core::clone::Clone for Arch {
    #[inline]
    fn clone(&self) -> Arch {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Arch {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Arch {
    #[inline]
    fn eq(&self, other: &Arch) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Arch {}
#[automatically_derived]
impl ::core::cmp::Eq for Arch {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Arch {
    #[inline]
    fn partial_cmp(
        &self,
        other: &Arch,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Arch {
    #[inline]
    fn cmp(&self, other: &Arch) -> ::core::cmp::Ordering {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        ::core::cmp::Ord::cmp(&__self_tag, &__arg1_tag)
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Arch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Arch::X86_64 => ::core::fmt::Formatter::write_str(f, "X86_64"),
            Arch::Aarch64 => ::core::fmt::Formatter::write_str(f, "Aarch64"),
        }
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::ValueEnum for Arch {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::X86_64, Self::Aarch64]
    }
    fn to_possible_value<'a>(
        &self,
    ) -> ::std::option::Option<clap::builder::PossibleValue> {
        match self {
            Self::X86_64 => Some({ clap::builder::PossibleValue::new("x86-64") }),
            Self::Aarch64 => Some({ clap::builder::PossibleValue::new("aarch64") }),
            _ => None,
        }
    }
}
pub enum Emulator {
    Qemu,
}
#[automatically_derived]
impl ::core::marker::Copy for Emulator {}
#[automatically_derived]
impl ::core::clone::Clone for Emulator {
    #[inline]
    fn clone(&self) -> Emulator {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Emulator {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Emulator {
    #[inline]
    fn eq(&self, other: &Emulator) -> bool {
        true
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Emulator {}
#[automatically_derived]
impl ::core::cmp::Eq for Emulator {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::cmp::PartialOrd for Emulator {
    #[inline]
    fn partial_cmp(
        &self,
        other: &Emulator,
    ) -> ::core::option::Option<::core::cmp::Ordering> {
        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
    }
}
#[automatically_derived]
impl ::core::cmp::Ord for Emulator {
    #[inline]
    fn cmp(&self, other: &Emulator) -> ::core::cmp::Ordering {
        ::core::cmp::Ordering::Equal
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Emulator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "Qemu")
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
)]
#[deny(clippy::correctness)]
impl clap::ValueEnum for Emulator {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Qemu]
    }
    fn to_possible_value<'a>(
        &self,
    ) -> ::std::option::Option<clap::builder::PossibleValue> {
        match self {
            Self::Qemu => Some({ clap::builder::PossibleValue::new("qemu") }),
            _ => None,
        }
    }
}
