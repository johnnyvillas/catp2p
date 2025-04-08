import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
  title: 'CatP2P',
  tagline: 'A high-performance peer-to-peer library for distributed computing',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://johnnyvillas.github.io',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, this is often '/<projectName>/'
  baseUrl: '/catp2p/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'johnnyvillas', // Usually your GitHub org/user name.
  projectName: 'catp2p', // Usually your repo name.
  trailingSlash: false,

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          // Please change this to your repo.
          editUrl:
            'https://github.com/johnnyvillas/catp2p/tree/main/docs-site/',
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          editUrl:
            'https://github.com/johnnyvillas/catp2p/tree/main/docs-site/',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    // Replace with your project's social card
    image: 'img/catp2p-social-card.jpg',
    navbar: {
      title: 'CatP2P',
      logo: {
        alt: 'CatP2P Logo',
        src: 'img/logo.svg',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'tutorialSidebar',
          position: 'left',
          label: 'Documentation',
        },
        {to: '/blog', label: 'Blog', position: 'left'},
        {
          href: 'https://github.com/johnnyvillas/catp2p',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Getting Started',
              to: '/docs/intro',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'GitHub Discussions',
              href: 'https://github.com/johnnyvillas/catp2p/discussions',
            },
            {
              label: 'Issues',
              href: 'https://github.com/johnnyvillas/catp2p/issues',
            },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'Blog',
              to: '/blog',
            },
            {
              label: 'GitHub',
              href: 'https://github.com/johnnyvillas/catp2p',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} CatP2P Project. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['rust', 'toml'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
