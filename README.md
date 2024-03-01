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

![quary_core_image](https://github.com/quarylabs/quary/assets/132601011/20024c62-6ad6-42e8-937e-37a708af9c0c)

## What is Quary?

Quary enables teams to model, test and deploy data transformations. This core is a fast and lightweight SQL transformation engine written in Rust.

- Visit [our website](https://www.quary.dev) to learn more
- Visit [our documentation](https://www.quary.dev/docs) to learn how to use Quary

## Understanding Quary

Teams use Quary to transform their raw data tables in a data warehouse into actionable trustworthy insights. Users can easily transform their data by writing SQL select statements, which Quary then converts into tables and views within the data warehouse.

A Quary project is formed by a collection of these select statements, also known as models. These models often build upon one another, creating a structured flow of data transformation. Quary simplifies the process of visualizing dependencies from the source (raw table) to the final insight. Additionally, it provides testing capabilities for the models at each stage, ensuring data integrity and accuracy.

## Getting Started

### Installation

```
brew install quarylabs/quary/quary
```

### Usage

Check out our template [here](https://github.com/quarylabs/template) of an example Quary project. The following commands clone the repo, and run some basic commands.

```
git clone https://github.com/quarylabs/template.git
cd template

quary compile
quary test -s
quary run
```

Note that you will most likely want to use our [Visual Studio Code extension](https://marketplace.visualstudio.com/items?itemName=Quary.quary-extension)

### Commands
```
quary help
```


## Support

If you run into any problems using Quary, please let us know. We want Quary to be easy-to-use, so if you are getting confused, it is our fault, not yours. [Create an issue](https://github.com/quarylabs/quary/issues) and we'll be happy to help you out.
