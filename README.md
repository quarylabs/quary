<p align="center">
  <a href="https://quary.dev">
    <picture>
      <img src="https://storage.googleapis.com/public_hosted_materials/quary.svg" height="128">
    </picture>
    <h1 align="center">quary</h1>
  </a>
</p>
<p align="center">
  <a aria-label="Quary logo" href="https://www.quary.dev/">
    <img src="https://img.shields.io/badge/MADE%20BY%20Quary-000000.svg?style=for-the-badge&logo=Quary&labelColor=000">
  </a>
  <a aria-label="Slack community" target="_blank" href="https://join.slack.com/t/quarylabs/shared_invite/zt-2dlbfnztw-dMLXJVL38NcbhqRuM5gUcw">
    <img src="https://img.shields.io/badge/slack-@quarycommunity-000000.svg?style=for-the-badge&logo=slack&labelColor=000" alt="Quary Community">
  </a>
  <a aria-label="License" href="https://github.com/quarylabs/quary/blob/main/LICENSE">
    <img alt="" src="https://img.shields.io/npm/l/next.svg?style=for-the-badge&labelColor=000000">
  </a>
</p>

![quary_core_image](./assets/diagram.jpg)

## What is Quary?

Quary enables teams to design, document, test and deploy data transformations to your SQL data stores. Quary is a SQL
engine and CLI tool that lets you manage your data transformation projects with ease.

Teams use Quary to transform their raw data tables into actionable and trustworthy insights. Teams do so by layering
well documented and tested transformations into useful insights ready for consumption. A Quary project is formed of a
collection of these SQL `SELECT` statements, that build on top of each other.

- Visit [our website](https://www.quary.dev) to learn more
- Visit [our documentation](https://www.quary.dev/docs) to learn how to use Quary

## Getting Started

### Installation

The following Quary repository contains the core Quary library as well as the Quary CLI tool. The Quary CLI tool can be
installed as following

#### Homebrew installation for macOS

Quary can be installed using Homebrew on macOS using the following command:

```
brew install quarylabs/quary/quary
```

#### Linux/Mac through curl

Quary can be installed using curl on Linux/Mac using the following command:

```shell
curl -fsSL https://raw.githubusercontent.com/quarylabs/quary/main/install.sh | bash
```

#### Other installations

Other builds are available in the [releases page](https://github.com/quarylabs/quary/releases/latest) to download.

### Usage

Once installed, a sample project can be created and run as follows:

```shell
mkdir example # create an empty project folder
cd example
quary init    # initialize DuckDB demo project with sample data
quary compile # validate the project structure and model references without database
quary build   # build and execute the model views/seeds against target database
quary test -s   # run defined tests against target database
```

Note that you will most likely want to use
our [Visual Studio Code extension](https://marketplace.visualstudio.com/items?itemName=Quary.quary-extension) to
visualise the project.

## Support

If you run into any problems using Quary, please let us know. We want Quary to be easy-to-use, so if you are getting
confused, it is our fault, not yours. [Create an issue](https://github.com/quarylabs/quary/issues) and we'll be happy to
help you out.
