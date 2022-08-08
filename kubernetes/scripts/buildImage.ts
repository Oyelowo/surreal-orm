import { INGRESS_EXTERNAL_PORT_LOCAL, hosts } from '../src/resources/infrastructure/ingress/hosts.js';

import sh from 'shelljs';

// sh.exec(`docker build --target web --build-arg NEXT_PUBLIC_API_URL=${INGRESS_EXTERNAL_PORT_LOCAL}`);
sh.exec(`docker build -f Dockerfile.development  \
                --target web -t $IMAGE \
                --build-arg NEXT_PUBLIC_API_URL=${hosts.local.apiUrl} .`)
                // --target web -t ghcr.io/oyelowo/react-web:d080afca-dirty \