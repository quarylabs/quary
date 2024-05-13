<h2>With Quary, engineers can:</h2>

- 🔌 Connect to their Database
- 📖 Write SQL queries to transform, organize, and document tables in a database
- 📊 Create charts, dashboards and reports (in development)
- 🧪 Test, collaborate & refactor iteratively through version control
- 🚀 Deploy the organised, documented model back up to the database

View the [documentation](https://www.quary.dev/docs).

## 🗃️ Supported Databases

<p align="center">
  <img src="https://img.shields.io/badge/Amazon%20Redshift-527FFF?style=for-the-badge&logo=Amazon%20Redshift&logoColor=white" alt="Amazon Redshift">
  <img src="https://img.shields.io/badge/Google%20BigQuery-4285F4?style=for-the-badge&logo=Google%20Cloud&logoColor=white" alt="Google BigQuery">
  <img src="https://img.shields.io/badge/PostgreSQL-336791?style=for-the-badge&logo=postgresql&logoColor=white" alt="PostgreSQL">
  <img src="https://img.shields.io/badge/Snowflake-29B5E8?style=for-the-badge&logo=snowflake&logoColor=white" alt="Snowflake">
  <img src="https://img.shields.io/badge/Supabase-3ECF8E?style=for-the-badge&logo=supabase&logoColor=white" alt="Supabase">
  <img src="https://img.shields.io/badge/DuckDB-FFF?style=for-the-badge&logo=duckdb&logoColor=black" alt="DuckDB">
  <img src="https://img.shields.io/badge/SQLite-003B57?style=for-the-badge&logo=sqlite&logoColor=white" alt="SQLite">
</p>

![quary_core_image](./assets/readme_demo.gif)

## 🏗️ Asset Types in Quary

Define and manage the following asset types as code:

- **Sources:** Define the external data sources that feed into Quary, such as database tables, flat files, or APIs (with DuckDB).
- **Models:** Transform raw data from sources into analysis-ready datasets using SQL, this lets engineers split complex queries into atomic components.
- **Charts:** Create visual representations of your data using SQL.
- **🚧 Dashboards (WIP):** Combine multiple charts into a single view, allowing engineers to monitor and analyze data in one place.
- **🚧 Reports (WIP):** Create detailed reports to share insights and findings with your team or stakeholders.

## 🚀 Getting Started

### Installation

Quary is a VSCode Extension (Interface) & Rust-based CLI (Core)

#### Extension

The VSCode extension can be installed [here](https://marketplace.visualstudio.com/items?itemName=Quary.quary-extension). Note that it depends on the CLI being installed.

#### CLI

#### Homebrew installation

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

<h2>🅀 Community</h2>

[Join our Slack channel](https://join.slack.com/t/quarylabs/shared_invite/zt-2dlbfnztw-dMLXJVL38NcbhqRuM5gUcw), for help, ideas, and discussions.

## Support

If you run into any problems using Quary, please let us know. We want Quary to be easy-to-use, so if you are getting
confused, it is our fault, not yours. [Create an issue](https://github.com/quarylabs/quary/issues) and we'll be happy to
help you out.

### Check out our other projects

[SQRUFF](https://github.com/quarylabs/sqruff), a compact, high-speed SQL linter, engineered with Rust efficiency.
