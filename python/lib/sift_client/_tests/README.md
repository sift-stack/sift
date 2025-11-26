# Sift Python

## Running Integration Tests Locally

1. Create Environmental Variables
   a. Create or open a .env file in /python
   b. Add an API key for SIFT_API_KEY
2. Start Local Sift
3. Asset Data: NostromoLV426
   a. Ensure that your local Sift instance contains data for the asset NostromoLV426
   b. If it doesn't them export data for NostromoLV426 from development
4. Run tests
   a. Run tests using /python/scripts/dev {test, test-integration, test-all}
