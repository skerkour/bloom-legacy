import store from '@/store';

export default function getApps() {
  const apps = [
  // {
  //   name: 'Chat',
  //   path: '/chat',
  //   icon: '/assets/imgs/icons/chat.svg',
  // },
    {
      name: 'Notes',
      path: '/notes',
      icon: '/assets/imgs/icons/notes.svg',
    },
    // {
    //   name: 'Music',
    //   path: '/music',
    //   icon: '/assets/imgs/icons/music.svg',
    // },
    {
      name: 'Calendar',
      path: '/calendar',
      icon: '/assets/imgs/icons/calendar.svg',
    },
    {
      name: 'Drive',
      path: '/drive',
      icon: '/assets/imgs/icons/drive.svg',
    },
    {
      name: 'Bitflow',
      path: '/bitflow',
      icon: '/assets/imgs/icons/bitflow.svg',
    },
    // {
    //   name: 'Gallery',
    //   path: '/gallery',
    //   icon: '/assets/imgs/icons/gallery.svg',
    // },
    {
      name: 'Arcade',
      path: '/arcade',
      icon: '/assets/imgs/icons/arcade.svg',
    },
    // {
    //   name: 'Books',
    //   path: '/books',
    //   icon: '/assets/imgs/icons/books.svg',
    // },
    {
      name: 'Contacts',
      path: '/contacts',
      icon: '/assets/imgs/icons/contacts.svg',
    },
    {
      name: 'Calculator',
      path: '/calculator',
      icon: '/assets/imgs/icons/calculator.svg',
    },
  ];

  if (store.state.me?.isAdmin) {
    apps.push({
      name: 'Admin',
      path: '/admin',
      icon: '/assets/imgs/icons/admin.svg',
    });
  }
  return apps;
}
