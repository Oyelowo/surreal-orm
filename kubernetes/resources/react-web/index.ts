import { provider } from '../shared/cluster';
import { ServiceDeployment } from '../shared/deployment';
import { reactWebSettings } from './settings';

export const reactWeb = new ServiceDeployment("react-web", reactWebSettings, {
  provider,
});
