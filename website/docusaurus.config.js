const lightCodeTheme = require("prism-react-renderer/themes/github")
const darkCodeTheme = require("prism-react-renderer/themes/dracula")

// With JSDoc @type annotations, IDEs can provide config autocompletion
/** @type {import('@docusaurus/types').DocusaurusConfig} */
module.exports = {
  title: "Moonwave",
  tagline: "Documentation Generator for Lua Projects",
  url: "https://upliftgames.github.io",
  baseUrl: "/moonwave/",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/favicon.ico",
  organizationName: "UpliftGames", // Usually your GitHub org/user name.
  projectName: "moonwave", // Usually your repo name.

  presets: [
    [
      "@docusaurus/preset-classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          // Please change this to your repo.
          editUrl:
            "https://github.com/UpliftGames/moonwave/edit/master/website/",
          showLastUpdateAuthor: true,
          showLastUpdateTime: true,
          sidebarCollapsible: true,
        },
        blog: {
          showReadingTime: true,
          editUrl:
            "https://github.com/UpliftGames/moonwave/edit/master/website/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        logo: {
          alt: "Moonwave",
          src: "img/logo.svg",
        },
        items: [
          {
            to: "docs/intro",
            position: "left",
            label: "Getting Started",
          },
          {
            to: "docs/TagList",
            position: "left",
            label: "List of Tags",
          },
          // { to: "/blog", label: "Blog", position: "left" },
          {
            href: "https://discord.gg/qaDRSjVNeq",
            label: "Discord",
            position: "right",
          },
          {
            href: "https://github.com/UpliftGames/moonwave",
            label: "GitHub",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        links: [
          {
            title: "Docs",
            items: [
              {
                label: "Getting Started with Moonwave",
                to: "/docs/intro",
              },
              {
                label: "List of Tags",
                to: "/docs/Tags",
              },
            ],
          },
          {
            title: "Community",
            items: [
              {
                label: "Discord",
                href: "https://discord.gg/qaDRSjVNeq",
              },
            ],
          },
          {
            title: "More",
            items: [
              // {
              //   label: "Blog",
              //   to: "/blog",
              // },
              {
                label: "GitHub",
                href: "https://github.com/UpliftGames/moonwave",
              },
              {
                label: "Uplift Games",
                href: "https://uplift.games",
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Uplift Games.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ["lua"],
      },
    }),
}
