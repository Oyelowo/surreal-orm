import crds from '../../../generatedCrdsTs/index.js';
import { metalbProvider } from './settings.js';

// TODO:
export const btpAdvert = new crds.metallb.v1beta1.BGPAdvertisement('bgpAdvert', {
    spec: {},
}, { provider: metalbProvider });
