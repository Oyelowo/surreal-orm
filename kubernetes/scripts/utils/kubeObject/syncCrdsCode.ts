import sh from 'shelljs';
import { getGeneratedCrdsCodeDir } from '../../../resources/shared/manifestsDirectory.js';
import { TCustomResourceDefinitionObject } from './kubeObject.js';

export function syncCrdsCode(crdKubeObjects: TCustomResourceDefinitionObject[]) {
    const manifestsCrdsFiles = crdKubeObjects.map(({ path }) => path);
    const outDir = getGeneratedCrdsCodeDir();
    sh.mkdir(outDir);

    sh.exec(` crd2pulumi --nodejsPath ${outDir} ${manifestsCrdsFiles.join(' ')} --force`);
    sh.exec(`npx prettier --write ${getGeneratedCrdsCodeDir()}`);
}
