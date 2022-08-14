import sh from 'shelljs';
import { getGeneratedCrdsCodeDir } from '../../../src/resources/shared/directoriesManager.js';
import { TKubeObject } from './kubeObject.js';

export function syncCrdsCode(crdKubeObjects: TKubeObject<'CustomResourceDefinition'>[]) {
    const manifestsCrdsFiles = crdKubeObjects.map(({ path }) => path);
    const outDir = getGeneratedCrdsCodeDir();
    sh.mkdir('-p', outDir);

    sh.exec(`crd2pulumi --nodejsPath ${outDir} ${manifestsCrdsFiles.join(' ')} --force`);
    sh.exec(`npx prettier --write ${getGeneratedCrdsCodeDir()}`);
}
