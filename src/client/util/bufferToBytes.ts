
// Useful for moving code to rust
export const dataBufferToHexBytes = (data: Buffer): string => {
  let msg = ''
  for (let i = 0; i < data.length; i++) {
    const element = data[i].toString(16)
    msg = msg + `,0x${element.length === 1 ? '0' + element : element}`
    // console.log(element.toString(16))
  }
  return msg
}