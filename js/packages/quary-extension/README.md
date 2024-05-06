# Quary VSCode Extension

Quary is a VSCode extension that streamlines data analysis within your editor. It allows you to organize, wrangle, and analyze data with ease, turning VSCode into a powerful data analysis workbench.

## Features

- **Seamless Integration**: Work with data right within VSCode.
- **Project Scaffolding**: Instantly generate a structured data project.
- **Model Visualization**: Render and visualize data models interactively.
- **Test Execution**: Run custom tests and validate your data models with built-in test suites.
- **Integrations**: Push your models and structure directly to a local SQLite database or BigQuery, allowing for collaboration and use by others.

## Requirements

- VSCode version 1.8 or higher.

## Core commands

- `quary.run`: Configure and run your Quary data project.
- `quary.runTests`: Execute tests against your data models.
- `quary.renderModel`: Visualize data models within VSCode.
  ...

## Known Issues

There are no known issues at the moment. For any new issues, please report them on our GitHub repository.

## Release Notes

- Initial prerelease version.

## Getting Started with Quary

Quary projects consist of seeds, models, tests, sources, and custom tests. Don't be daunted by the terminology; our guide will walk you through each component.

### Quickstart

1. **Open the Sample Project**: Access our template repository in GitHub's VSCode instance with no account required.
2. **Install Quary**: Search for `Quary.quary` in the extensions tab and install.
3. **Run Your Project**: Use `QUARY: Run` from the command palette to start your project against a sqlite database.

### Anatomy of a Quary Project

- `project.yml`: Root configuration file defining tests, models, and seeds.
- `.quary.config.yaml`: Specifies build configurations and database connections.
- `seeds/`: Holds initial data from .csv files to populate the database.
- `models/`: Contains SQL-based data models for project analysis.

### Your First Model

Models are at the heart of Quary, letting you transform raw data into insightful analytics. For example, `shifts_by_month` helps analyze employee shifts over time.

```sql
SELECT
  employee_id,
  strftime('%Y-%m', shift_date) AS shift_month,
  COUNT(*)                     AS total_shifts
FROM q.stg_shifts
GROUP BY employee_id, shift_month
```
