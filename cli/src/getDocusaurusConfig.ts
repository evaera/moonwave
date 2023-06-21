import fs from "fs-extra"
import path from "path"
import { ClassOrder, Config, FoldersEnabled } from "./prepareProject.js"

export interface GenerateConfigParams {
  codePaths: string[]
  enablePlugins: FoldersEnabled
  config: Config
  customCssExists: boolean
  customSidebarExists: boolean
  changelogExists: boolean
  projectDir: string
  binaryPath: string
  classOrder: ClassOrder
  apiCategories: string[]
  autoSectionPath?: string
}

export default function getDocusaurusConfig({
  codePaths,
  enablePlugins,
  config,
  customCssExists,
  customSidebarExists,
  changelogExists,
  projectDir,
  binaryPath,
  classOrder,
  apiCategories,
  autoSectionPath,
}: GenerateConfigParams) {
  const gitRepoUrl = config.gitRepoUrl

  const validCodePaths = codePaths
    .map((codePath) => path.join(process.cwd(), codePath))
    .filter((codePath) => fs.existsSync(codePath))

  return {
    onBrokenLinks: "throw",
    onBrokenMarkdownLinks: "warn",
    url: `https://${config.docusaurus?.organizationName}.github.io`,

    ...config.docusaurus,

    customFields: {
      bannerImage: config.home?.bannerImage
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
          ...(config?.navbar?.items || []),
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
        copyright: `Copyright © ${new Date().getFullYear()} ${
          config.docusaurus?.organizationName ?? ""
        }. Built with Moonwave and Docusaurus.`,
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
          sourceUrl: gitRepoUrl + `/blob/${config.gitSourceBranch ?? "master"}`,
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
                  ? `${gitRepoUrl}/edit/${config.gitSourceBranch ?? "master"}/`
                  : undefined, // Omitting this variable entirely will disable edit links
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
                  ? `${gitRepoUrl}/edit/${config.gitSourceBranch ?? "master"}/`
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
  }
}
