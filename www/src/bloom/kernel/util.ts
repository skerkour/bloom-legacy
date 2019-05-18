class Util {
  sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  download_file(url: string) {
    const download_link: any = document.createElement('a');
    download_link.href = url;
    document.body.appendChild(download_link);
    download_link.click();
    document.body.removeChild(download_link);
  }
}

export default new Util();
