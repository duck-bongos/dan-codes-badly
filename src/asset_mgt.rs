use dioxus::prelude::*;

pub const TOTODILE: Asset = asset!(
    "/assets/totodile.png",
    // You can pass a second argument to the asset macro to set up options for the asset
    ImageAssetOptions::new()
        // You can set the image size in pixels at compile time to send the smallest possible image to the client
        .with_size(ImageSize::Automatic)
        // You can also convert the image to a web friendly format at compile time. This can make your images significantly smaller
        .with_format(ImageFormat::Png)
);

pub const BOSTON: Asset = asset!(
    "/assets/img/boston.jpg",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Jpg)
);

pub const CINCINNATI: Asset = asset!(
    "/assets/img/cincinnati.jpeg",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Jpg)
);

pub const EGGS: Asset = asset!(
    "/assets/img/eggs.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Png)
);

pub const MIRADOR_CONDOR: Asset = asset!(
    "/assets/img/header_image.jpg",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Jpg)
);

pub const LINDNER: Asset = asset!(
    "/assets/img/lindner.jpg",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Jpg)
);

pub const LOGO: Asset = asset!(
    "/assets/img/logo.jpg",
    ImageAssetOptions::new().with_format(ImageFormat::Jpg)
);

pub const PHILLY: Asset = asset!(
    "/assets/img/philly.jpg",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Jpg)
);
pub const PRINCETON: Asset = asset!(
    "/assets/img/princeton.webp",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Webp)
);

pub const PROFILE: Asset = asset!(
    "/assets/img/profile.gif",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 256,
            height: 256
        })
        
);

pub const SBU: Asset = asset!(
    "/assets/img/sbu.jpg",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Jpg)
);

pub const UC: Asset = asset!(
    "/assets/img/uc.webp",
    ImageAssetOptions::new()
        .with_size(ImageSize::Automatic)
        .with_format(ImageFormat::Webp)
);
