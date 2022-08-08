export function toBase64(text: string): string {
    return Buffer.from(text).toString('base64');
}
