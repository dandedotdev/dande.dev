/**
 * ### Conventional Commits configuration
 *
 * https://www.npmjs.com/package/git-cz
 * https://www.conventionalcommits.org/en/v1.0.0/
 */

module.exports = {
  disableEmoji: true,
  format: '{type}{scope}: {emoji}{subject}',
  list: [
    'feat',
    'fix',
    'docs',
    'style',
    'refactor',
    'perf',
    'test',
    'build',
    'ci',
    'chore',
    'release',
  ],
  maxMessageLength: 64,
  minMessageLength: 3,
  questions: ['type', 'scope', 'subject', 'body', 'breaking', 'issues', 'lerna'],
  scopes: [
    // the changes across crates (e.g. Dockerfile, LICENSE, CHANGELOG.md, README.md, etc.)
    '*',
    // app
    'app',
    // client
    'client',
    // configurations (e.g. .cargo, .cspell.config.yaml, .github, .vscode, rust-toolchain.toml...)
    'config',
    // domain
    'domain',
    // developer experience (.editorconfig, .husky, .taplo...)
    'dx',
    // e2e tests
    'end2end',
    // generator
    'generator',
    // packages (e.g. crates, npm, pnpm...)
    'packages',
    // post (things related to post, e.g. data and public.)
    'post',
    // public
    'public',
    // styles (e.g. styles, public/styles)
    'styles',
    // ui
    'ui',
  ],
  types: {
    feat: {
      description: 'A new feature',
      emoji: '🎸',
      value: 'feat',
    },
    fix: {
      description: 'A bug fix',
      emoji: '🐛',
      value: 'fix',
    },
    docs: {
      description: 'Documentation only changes',
      emoji: '✏️',
      value: 'docs',
    },
    style: {
      description:
        'Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)',
      emoji: '💄',
      value: 'style',
    },
    refactor: {
      description: 'A code change that neither fixes a bug or adds a feature',
      emoji: '💡',
      value: 'refactor',
    },
    perf: {
      description: 'A code change that improves performance',
      emoji: '⚡️',
      value: 'perf',
    },
    test: {
      description: 'Adding missing tests or correcting existing tests',
      emoji: '💍',
      value: 'test',
    },
    build: {
      description:
        'Changes that affect the build system or external dependencies (example scopes: Cargo, Docker)',
      emoji: '🧱',
      value: 'build',
    },
    ci: {
      description: 'Changes to our CI configuration files and scripts (example scopes: Drone)',
      emoji: '🎡',
      value: 'ci',
    },
    chore: {
      description: "Other changes that don't modify src or test files",
      emoji: '🤖',
      value: 'chore',
    },
    release: {
      description: 'Create a release commit',
      emoji: '🏹',
      value: 'release',
    },
    messages: {
      type: "Select the type of change that you're committing:",
      customScope: 'Select the scope this component affects:',
      subject: 'Write a short, imperative mood description of the change:\n',
      body: 'Provide a longer description of the change:\n ',
      breaking: 'List any breaking changes:\n',
      footer: 'Issues this commit closes, e.g #123:',
      confirmCommit: 'The packages that this commit has affected\n',
    },
  },
};
