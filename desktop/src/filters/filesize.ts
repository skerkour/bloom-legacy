import filesize from '@/libs/filesize';

export default function (value: number) {
  return filesize(value, { base: 10 });
}
