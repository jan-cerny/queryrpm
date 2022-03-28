use crate::rpm;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

pub fn get_nvr(name: &str) -> Option<String> {
    unsafe {
        if rpm::rpmReadConfigFiles(ptr::null(), ptr::null()) != 0 {
            return None;
        }
        let rpmts = rpm::rpmtsCreate();
        let name_cstring = CString::new(name).unwrap();
        let mi = rpm::rpmtsInitIterator(
            rpmts,
            rpm::rpmTag_e_RPMTAG_NAME,
            name_cstring.as_ptr() as *mut _,
            0,
        );
        let count = rpm::rpmdbGetIteratorCount(mi);
        if count != 0 {
            let header = rpm::rpmdbNextIterator(mi);
            let errstr = CString::new("").unwrap();
            let fmt_cstring = CString::new("%{NAME}-%{VERSION}-%{RELEASE}.%{ARCH}").unwrap();
            let name_ptr =
                rpm::headerFormat(header, fmt_cstring.as_ptr(), errstr.as_ptr() as *mut _);
            let name = CStr::from_ptr(name_ptr);
            return Some(String::from(name.to_str().unwrap()));
        }
        rpm::rpmtsFree(rpmts);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nvr_query() {
        assert_eq!(
            get_nvr("openscap"),
            Some(String::from("openscap-1.3.6-3.fc35.x86_64"))
        );
        assert_eq!(get_nvr("smazeny_syr"), None);
    }
}
