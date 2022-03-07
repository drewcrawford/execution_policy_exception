/*! Micro bindings for executionpolicy framework.

I am not vending this as a lib as I suspect nobody, rust or otherwise, will ever use it who isn't me

If you are reading this, send me an email @ drew@sealedabstract.com with your usecase!
*/

use std::os::raw::c_long;
use objr::bindings::*;
use foundationr::NSURL;
objc_enum! {
    #[derive(Debug,PartialEq,Eq)]
    pub struct EPDeveloperToolStatus<c_long>;
    impl EPDeveloperToolStatus {
    NotDetermined = 0,
    Restricted = 1,
    Denied = 2,
    Authorized = 3
    }
}

objc_class! {
    pub struct EPDeveloperTool {
        @class(EPDeveloperTool)
    }
}
objc_selector_group! {
    trait Selectors {
        @selector("authorizationStatus")
        @selector("requestDeveloperToolAccessWithCompletionHandler:")
    }
    impl Selectors for Sel {}
}

blocksr::once_escaping!(RequestAccess(granted: bool) -> ());
unsafe impl Arguable for &RequestAccess {}

#[allow(non_snake_case)]
impl EPDeveloperTool {
    pub fn init(pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        Self::class().alloc_init(pool)
    }
    pub fn authorizationStatus(&self, pool: &ActiveAutoreleasePool) -> EPDeveloperToolStatus {
        unsafe {
            EPDeveloperToolStatus(Self::perform_primitive(self.assume_nonmut_perform(),Sel::authorizationStatus(), pool, () ))
        }
    }
    pub fn requestAccess<C: Fn(bool) + Send + 'static>(&self, completion: C,pool: &ActiveAutoreleasePool) {
        let block = unsafe{RequestAccess::new(completion)};

        unsafe {
            Self::perform_primitive(self.assume_nonmut_perform(), Sel::requestDeveloperToolAccessWithCompletionHandler_(), pool, (&block,))
        }
    }
}

objc_class! {
    pub struct EPExecutionPolicy {
        @class(EPExecutionPolicy)
    }
}

objc_selector_group! {
    trait Selectors2 {
        @selector("addPolicyExceptionForURL:error:")
    }
    impl Selectors2 for Sel {}
}

#[allow(non_snake_case)]
impl EPExecutionPolicy {
    pub fn init(pool: &ActiveAutoreleasePool) -> StrongCell<Self> {
        Self::class().alloc_init(pool)
    }
    pub fn addPolicyExceptionForURLError<'a>(&self, url: &NSURL,pool: &'a ActiveAutoreleasePool) -> Result<(),AutoreleasedCell<'a, NSError>> {
        unsafe {
            Self::perform_bool_result(self.assume_nonmut_perform(), Sel::addPolicyExceptionForURL_error(),pool, (url,) )
        }
    }
}


#[link(name="ExecutionPolicy",kind="framework")]
extern "C" {

}
