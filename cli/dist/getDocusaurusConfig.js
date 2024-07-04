import fs from "fs-extra";
import path from "path";
export default function getDocusaurusConfig({ codePaths, enablePlugins, config, customCssExists, customSidebarExists, changelogExists, projectDir, binaryPath, classOrder, apiCategories, autoSectionPath, }) {
    var _a, _b, _c, _d, _e, _f, _g, _h;
    const gitRepoUrl = config.gitRepoUrl;
    const validCodePaths = codePaths
        .map((codePath) => path.join(process.cwd(), codePath))
        .filter((codePath) => fs.existsSync(codePath));
    return {
        onBrokenLinks: "throw",
        onBrokenMarkdownLinks: "warn",
        url: `https://${(_a = config.docusaurus) === null || _a === void 0 ? void 0 : _a.organizationName}.github.io`,
        ...config.docusaurus,
        customFields: {
            bannerImage: (_b = config.home) === null || _b === void 0 ? void 0 : _b.bannerImage
        },
        themeConfig: {
            prism: {
                additionalLanguages: ["lua"],
            },
            navbar: {
                ...config.navbar,
                items: [
                    ...(enablePlugins.docs
                        ? [
                            {
                                type: "doc",
                                docId: "intro",
                                position: "left",
                                label: "Docs",
                            },
                        ]
                        : []),
                    ...(enablePlugins.blog
                        ? [{ to: "/blog", label: "Blog", position: "left" }]
                        : []),
                    ...(validCodePaths.length > 0
                        ? [{ to: "/api/", label: "API", position: "left" }]
                        : []),
                    ...(changelogExists
                        ? [{ to: "/changelog", label: "Changelog", position: "left" }]
                        : []),
                    ...(((_c = config === null || config === void 0 ? void 0 : config.navbar) === null || _c === void 0 ? void 0 : _c.items) || []),
                    ...(gitRepoUrl
                        ? [
                            {
                                href: gitRepoUrl,
                                label: "GitHub",
                                position: "right",
                            },
                        ]
                        : []),
                ],
            },
            footer: {
                style: "dark",
                copyright: `Copyright © ${new Date().getFullYear()} ${(_e = (_d = config.docusaurus) === null || _d === void 0 ? void 0 : _d.organizationName) !== null && _e !== void 0 ? _e : ""}. Built with Moonwave and Docusaurus.`,
                ...config.footer,
            },
            colorMode: {
                respectPrefersColorScheme: true,
            },
        },
        plugins: [
            [
                "docusaurus-plugin-moonwave",
                {
                    id: "moonwave",
                    code: validCodePaths,
                    sourceUrl: gitRepoUrl + `/blob/${(_f = config.gitSourceBranch) !== null && _f !== void 0 ? _f : "master"}`,
                    projectDir,
                    classOrder,
                    apiCategories,
                    binaryPath,
                    autoSectionPath,
                },
            ],
            "docusaurus-lunr-search",
        ],
        presets: [
            [
                "@docusaurus/preset-classic",
                {
                    docs: enablePlugins.docs
                        ? {
                            // Please change this to your repo.
                            editUrl: gitRepoUrl
                                ? `${gitRepoUrl}/edit/${(_g = config.gitSourceBranch) !== null && _g !== void 0 ? _g : "master"}/`
                                : undefined,
                            sidebarCollapsible: true,
                            sidebarPath: customSidebarExists
                                ? "./src/sidebars.js"
                                : undefined,
                        }
                        : false,
                    blog: enablePlugins.blog
                        ? {
                            showReadingTime: true,
                            // Please change this to your repo.
                            editUrl: gitRepoUrl
                                ? `${gitRepoUrl}/edit/${(_h = config.gitSourceBranch) !== null && _h !== void 0 ? _h : "master"}/`
                                : undefined, // Omitting this variable entirely will disable edit links
                        }
                        : false,
                    pages: {
                        path: "pages",
                        //exclude any file starting with an underscore
                        exclude: ["_*.*"],
                    },
                    theme: {
                        customCss: [
                            "src/css/moonwave.css",
                            ...(customCssExists ? ["src/css/custom.css"] : []),
                        ],
                    },
                },
            ],
        ],
    };
}
//# sourceMappingURL=getDocusaurusConfig.js.map