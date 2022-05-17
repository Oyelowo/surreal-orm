import { getResourceProvider } from '../shared/manifestsDirectory'
import { getEnvironmentVariables } from '../shared/validations'

const { ENVIRONMENT } = getEnvironmentVariables()
export const namespacesNamesProvider = getResourceProvider('namespace-names', ENVIRONMENT)
