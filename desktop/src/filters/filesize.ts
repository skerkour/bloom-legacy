import filesize from '@/libs/filesize';

export default function fileSize(value: number) {
  return filesize(value, { base: 10 });
}
