import * as linode from '@pulumi/linode';
// import * as pulumi from '@pulumi/pulumi';

export const domain = new linode.Domain('domain-production', {
    domain: 'oyelowo.dev',
    soaEmail: 'example@oyelowo.dev',
    type: 'master',
    // refreshSec
    // masterIps
});


// export const domainR = new linode.DomainRecord('domain-production-record', {
//     domainId: domain.id,
//     name: "www",
//     recordType: "CNAME",
//     target: 'oyelowo.dev',
//     // refreshSec
//     // masterIps
// });
