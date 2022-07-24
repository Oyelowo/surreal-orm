import sh from 'shelljs';
import { getGeneratedCrdsCodeDir, getMainBaseDir } from '../../../resources/shared/manifestsDirectory';
import path from 'path';
import { TCustomResourceDefinitionObject } from './kubeObject';


export function syncCrdsCode(crdKubeObjects: TCustomResourceDefinitionObject[]) {
    const manifestsCrdsFiles = crdKubeObjects.map(({ path }) => path);
    const outDir = path.join(getMainBaseDir(), 'crds-generated');

    sh.exec(` crd2pulumi --nodejsPath ${outDir} ${manifestsCrdsFiles.join(' ')} --force`);
    sh.exec(`npx prettier --write ${getGeneratedCrdsCodeDir()}`);
}
