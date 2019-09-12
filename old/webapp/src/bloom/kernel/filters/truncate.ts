export default function(text: string, length = 42, suffix = '...') {
  return `${text.substring(0, length)}${suffix}`;
}
