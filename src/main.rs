/*!
Tired of getting 'App is damaged and can’t be opened. You should move it to the Trash' when building an app for local development?

Xcode requests a 'system policy exception' for its applications by using the `builtin-RegisterExecutionPolicyException` build step.
However, as you can tell by the name, there is no normal Unix command to do this, the logic is 'builtin' to Xcode and its build
system and cannot be run separately.

This is a standalone Unix utility that does the same thing.  Like Xcode, it requires the 'Developer Tools' security permission.
*/
mod executionpolicy;

use std::path::PathBuf;
use std::sync::mpsc::sync_channel;
use clap::Parser;
use objr::bindings::{ActiveAutoreleasePool, autoreleasepool};
use crate::executionpolicy::{EPDeveloperTool,EPDeveloperToolStatus,EPExecutionPolicy};
use foundationr::NSURL;

#[derive(Parser)]
#[clap(name="execution_polcy_exception")]
#[clap(author="Drew Crawford <drew@sealedabstract.com>")]
/**
Requests a system policy exception with ExceptionPolicy framework.

This software is free for noncommercial and 'small commercial' use.
*/
struct Cli {
    /**
    The path of executable for which we will request an exception.
    */
    path: PathBuf
}
fn developer_tools_issue() {
    panic!("Failed to get access to developer tools.

    Open System Preferences, access Security tab, access Privacy tab, scroll down to 'Developer Tools', and ensure that `execution_policy_exception`,
    or the application using it (such as Terminal, etc.) is checked.

    This will allow execution_policy_exception to work, e.g. to grant exceptions to run arbitrary code.

    By the way, you should only use or install this tool if you are building unsigned code for local development.
     ")
}
fn authorize_maybe(developer_tool: &EPDeveloperTool,pool: &ActiveAutoreleasePool) {
    let (sender,receiver) = sync_channel(1);
    developer_tool.requestAccess(move |granted| {
        sender.send(granted).unwrap();
    }, pool);
    let result = receiver.recv().unwrap();
    if !result {
        developer_tools_issue()
    }
}
fn main() {
    let args = Cli::parse();
    autoreleasepool(|pool| {
        let developer_tool = executionpolicy::EPDeveloperTool::init(pool);
        let status = developer_tool.authorizationStatus(pool);

        match status {
            EPDeveloperToolStatus::NotDetermined | EPDeveloperToolStatus::Denied | EPDeveloperToolStatus::Restricted => {
                authorize_maybe(&developer_tool,pool)
            }
            EPDeveloperToolStatus::Authorized => { /* */ }
            other => {
                todo!("Unsupported developer tools status {other:?}")
            }
        }
        use foundationr::NSStringExtension;
        let p = &args.path.into_os_string().into_string().unwrap();
        let path_nsstring = objr::foundation::NSString::from_borrowed_str(&p,pool);
        let url = NSURL::initFileURLWithPath(&path_nsstring, pool);
        let result = EPExecutionPolicy::init(pool).addPolicyExceptionForURLError(&url, pool);
        match result {
            Ok(()) => {},
            Err(e) => {
                panic!("Failed to get system policy exception for {url} due to {e}");
            }
        }
    })

}

