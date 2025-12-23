# Problem Description

The core challenge this application addresses is the **Profitability Analysis of Mobile Applications**.
Developing a mobile app involves a complex set of variable costs—from device procurement and server infrastructure to mixed-team salaries (Developers, QA). Without a unified calculation engine, it is difficult to accurately predict whether a project will be profitable or to determine its break-even point.

## Key Features & Use Cases

1.  **Profitability & Cost Structure Calculation**
    *   Aggregates all cost drivers: hardware (devices, servers) and human capital (developer/QA salaries).
    *   Provides a precise "Investment Cost" baseline which is the denominator for any Return on Investment (ROI) or profitability formula.

2.  **Comparative Financial Analysis**
    *   Allows stakeholders to compare different financial scenarios.
    *   Example: Comparing infrastructure costs vs. personnel costs (e.g., `server_price` vs `qa_salary`) or comparing the costs of two different deployment strategies to maximize margin.

3.  **Sensitivity & Risk Assessment**
    *   Analyzes how volatility in specific cost factors (e.g., a rise in `server_price`) impacts the overall financial model.
    *   Helps in stress-testing the business model against potential market changes.

## Technical Solution

The service provides a REST API (`/api/calc`) serving as the financial brain for the mobile project.

### Supported Operations
*   `calculate`: Computes the Total Cost of Ownership (TCO) based on resource counts and unit prices.
*   `compare`: Evaluates two financial metrics (e.g. `server_price`, `qa_salary`) to determine the optimal allocation of funds.
*   `sensitivity`: Projects the impact of cost fluctuations on a given variable (e.g. `server_price`).
