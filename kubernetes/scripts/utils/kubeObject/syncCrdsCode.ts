import sh from 'shelljs';
import { getGeneratedCrdsCodeDir, getMainBaseDir } from '../../../resources/shared/manifestsDirectory.js';
import path from 'path';
import { TCustomResourceDefinitionObject } from './kubeObject.js';

export function syncCrdsCode(crdKubeObjects: TCustomResourceDefinitionObject[]) {
    const manifestsCrdsFiles = crdKubeObjects.map(({ path }) => path);
    const outDir = path.join(getMainBaseDir(), 'generatedCrdsTs');
    sh.mkdir(outDir);

    sh.exec(` crd2pulumi --nodejsPath ${outDir} ${manifestsCrdsFiles.join(' ')} --force`);
    sh.exec(`npx prettier --write ${getGeneratedCrdsCodeDir()}`);
}
