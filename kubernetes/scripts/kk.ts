import { getEnvVarsForKubeManifests, kubeBuildEnvVarsManager } from '../src/resources/types/environmentVariables.js';
import * as dotenv from 'dotenv';

kubeBuildEnvVarsManager.syncAll()
// kubeBuildEnvVarsManager.syncAll()

// dotenv.config({ path:  });

// const env = getEnvVarsForKubeManifestGenerator()

// console.log('env', env);