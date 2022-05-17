import * as pulumi from '@pulumi/pulumi'

const config = new pulumi.Config('oye')

const isLocal = config.require('isLocal')
