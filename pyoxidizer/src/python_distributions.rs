// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Defines known Python distributions.

use {
    crate::py_packaging::distribution::{
        DistributionFlavor, PythonDistributionLocation, PythonDistributionRecord,
    },
    itertools::Itertools,
    once_cell::sync::Lazy,
};

pub struct PythonDistributionCollection {
    dists: Vec<PythonDistributionRecord>,
}

impl PythonDistributionCollection {
    /// Find a Python distribution given requirements.
    ///
    /// `target_triple` is the Rust machine triple the distribution is built for.
    /// `flavor` is the type of Python distribution.
    /// `python_major_minor_version` is an optional `X.Y` version string being
    /// requested. If `None`, `3.9` is assumed.
    pub fn find_distribution(
        &self,
        target_triple: &str,
        flavor: &DistributionFlavor,
        python_major_minor_version: Option<&str>,
    ) -> Option<PythonDistributionRecord> {
        let python_major_minor_version = python_major_minor_version.unwrap_or("3.9");

        self.dists
            .iter()
            .filter(|dist| dist.python_major_minor_version == python_major_minor_version)
            .filter(|dist| dist.target_triple == target_triple)
            .filter(|dist| match flavor {
                DistributionFlavor::Standalone => true,
                DistributionFlavor::StandaloneStatic => !dist.supports_prebuilt_extension_modules,
                DistributionFlavor::StandaloneDynamic => dist.supports_prebuilt_extension_modules,
            })
            .cloned()
            .next()
    }

    /// Obtain records for all registered distributions.
    #[allow(unused)]
    pub fn iter(&self) -> impl Iterator<Item = &PythonDistributionRecord> {
        self.dists.iter()
    }

    /// All target triples of distributions in this collection.
    #[allow(unused)]
    pub fn all_target_triples(&self) -> impl Iterator<Item = &str> {
        self.dists
            .iter()
            .map(|dist| dist.target_triple.as_str())
            .sorted()
            .dedup()
    }
}

pub static PYTHON_DISTRIBUTIONS: Lazy<PythonDistributionCollection> = Lazy::new(|| {
    let dists = vec![
        // Linux glibc linked.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.8.12-x86_64-unknown-linux-gnu-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "93b0a95af0dab42b2214e381efd183acea1db47bf26b304fdfd625b32a8ae443".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.9.10-x86_64-unknown-linux-gnu-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "1cb15d524deea85c7a0c7cd6e4a90b99472b44dfafd095a2429c9b583927e315".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.10.2-x86_64-unknown-linux-gnu-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "7bc1d7ed193c984a8937797d6fa02e26447bbde835be0334e9730d3e441f0cb2".to_string(),
            },
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // Linux musl.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.8.12-x86_64-unknown-linux-musl-noopt-20220220T1113.tar.zst".to_string(),
                sha256: "3b716121ae8d1e2e47ab8cebe0381e9c2e89438252ea8ebd0932537e8bc06320".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.9.10-x86_64-unknown-linux-musl-noopt-20220220T1113.tar.zst".to_string(),
                sha256: "ff2806064909057f4b29fbef533b80fb3330940f4c1e89157e548a1677454cb2".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.10.2-x86_64-unknown-linux-musl-noopt-20220220T1113.tar.zst".to_string(),
                sha256: "7d9376b77881ad29dc828fbdebe7631b3c7896e34f278145f7fc4a7a5d5f25c5".to_string(),
            },
            target_triple: "x86_64-unknown-linux-musl".to_string(),
            supports_prebuilt_extension_modules: false,
        },

        // The order here is important because we will choose the
        // first one. We prefer shared distributions on Windows because
        // they are more versatile: statically linked Windows distributions
        // don't declspec(dllexport) Python symbols and can't load shared
        // shared library Python extensions, making them a pain to work
        // with.

        // Windows shared.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.8.12-i686-pc-windows-msvc-shared-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "5682c6c9f1378b8a05a7cc40d00dbd843a66b2f6798ac04d2487952860f2c15a".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.9.10-i686-pc-windows-msvc-shared-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "c2d5d0d54469f2f2cfc8b728fb7ef05860917930552f546c94cfa6f7f95f146d".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.10.2-i686-pc-windows-msvc-shared-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "38ef98a92b7e5cfb1d83ed2d4493d668a25558a4774cd57705ab013e9365e06a".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.8.12-x86_64-pc-windows-msvc-shared-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "0ddddc3e497ea07731ad80846ac365955c339db5abedad4677130b9a7976b7be".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.9.10-x86_64-pc-windows-msvc-shared-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "25afcb7e1732ac2201fde8e25409de4a09d765e97f750e2173884c04bb1c03c6".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.10.2-x86_64-pc-windows-msvc-shared-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "791f5d3038a5bd3b2933e21e9c5a5d2cd21a059174857a4257b1953c94acf7b7".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: true,
        },

        // Windows static.
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.8.12-i686-pc-windows-msvc-static-noopt-20220220T1113.tar.zst".to_string(),
                sha256: "d871f64c0efc44e579d0d214c5da7bb50a39a5c3fd75a52184cda26ca4fa1104".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.9.10-i686-pc-windows-msvc-static-noopt-20220220T1113.tar.zst".to_string(),
                sha256: "07e0b93b66fb7f2dec0a10c114de1b18908808915a91f65dff2fc7e19a839199".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.10.2-i686-pc-windows-msvc-static-noopt-20220220T1113.tar.zst".to_string(),
                sha256: "cec51f727e1e1f7e5f11a2b4e6cb7120296339c1fe018e3be7aa5c0df686de65".to_string(),
            },
            target_triple: "i686-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.8.12-x86_64-pc-windows-msvc-static-noopt-20220220T1113.tar.zst".to_string(),
                sha256: "975a075f2a9a20cc9fa43b64422e44136947da14a2a0cabe589a0ce020bbab36".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.9.10-x86_64-pc-windows-msvc-static-noopt-20220220T1113.tar.zst".to_string(),
                sha256: "b8de8f187612a40f190f3c2527d5b1578861ed7bf215fcf74992730a1effc68f".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.10.2-x86_64-pc-windows-msvc-static-noopt-20220220T1113.tar.zst".to_string(),
                sha256: "fd17035c8861ebda993ebd4cfda7f53138f19c570a179c3c90bee41190733978".to_string(),
            },
            target_triple: "x86_64-pc-windows-msvc".to_string(),
            supports_prebuilt_extension_modules: false,
        },

        // macOS.
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.9.10-aarch64-apple-darwin-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "179a3e81823b8b0614a9385a2161a9fb3c60f8f26816cac1dc1eb03cfee81d04".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.10.2-aarch64-apple-darwin-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "ae5544c65b1ef8380c7b7d74c3a2e6d6596b53cccfbe857f0bd67fc3823c2e85".to_string(),
            },
            target_triple: "aarch64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.8".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.8.12-x86_64-apple-darwin-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "5255967cf0484269aca6d84e3b79ad948d3899f7c00ce86b53629ab2786317ec".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.9".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.9.10-x86_64-apple-darwin-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "a527b9ca311f4ea48efde038274554d6b5aada6ad2e454a25ceac5733fdd13bb".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
        PythonDistributionRecord {
            python_major_minor_version: "3.10".to_string(),
            location: PythonDistributionLocation::Url {
                url: "https://github.com/indygreg/python-build-standalone/releases/download/20220222/cpython-3.10.2-x86_64-apple-darwin-pgo-20220220T1113.tar.zst".to_string(),
                sha256: "200b51c4120558cc643834a1d90ced80e596e39faca9c5610e6e789b7f7453da".to_string(),
            },
            target_triple: "x86_64-apple-darwin".to_string(),
            supports_prebuilt_extension_modules: true,
        },
    ];
    PythonDistributionCollection { dists }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_target_triples() {
        assert_eq!(
            PYTHON_DISTRIBUTIONS
                .all_target_triples()
                .collect::<Vec<_>>(),
            vec![
                "aarch64-apple-darwin",
                "i686-pc-windows-msvc",
                "x86_64-apple-darwin",
                "x86_64-pc-windows-msvc",
                "x86_64-unknown-linux-gnu",
                "x86_64-unknown-linux-musl",
            ]
        );
    }
}
