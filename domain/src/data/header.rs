use crate::types::HeaderNavLinks;

pub const HEADER_NAV_LINKS: [HeaderNavLinks<'_>; 2] = [
    HeaderNavLinks {
        href: "/blog",
        label: "Blog",
    },
    // HeaderNavLinks {
    //     href: "/tags",
    //     label: "Tags",
    // },
    HeaderNavLinks {
        href: "/about",
        label: "About",
    },
];
