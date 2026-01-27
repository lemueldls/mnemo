declare global {
  interface Uint8Array {
    toBase64(): string;
  }

  interface Uint8ArrayConstructor {
    fromBase64(this: this, base64: string): Uint8Array;
  }
}

if (!Uint8Array.prototype.toBase64) {
  Uint8Array.prototype.toBase64 = function toBase64() {
    let binaryString = "";
    for (let i = 0; i < this.length; i++) binaryString += String.fromCharCode(this[i]!);

    return btoa(binaryString);
  };
}

if (!Uint8Array.fromBase64) {
  Uint8Array.fromBase64 = function fromBase64(base64String) {
    // Decode the Base64 string to a string of binary data
    const binaryString = atob(base64String);
    // Create a Uint8Array from the binary string
    const len = binaryString.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) bytes[i] = binaryString.charCodeAt(i);

    return bytes;
  };
}

export * from "../lib/storage/items";
export * from "../lib/storage/keys";
