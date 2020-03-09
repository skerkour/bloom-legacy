const {
  app,
  BrowserWindow,
  Tray,
  Menu,
} = require('electron');
const config = require('./config');


// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let mainWindow = null;
let tray = null;

// show window
function showWindow() {
  // const position = getWindowPosition();
  // mainWindow.setPosition(position.x, position.y, false);
  mainWindow.show();
  mainWindow.focus();
}

// toggle window
function toggleWindow() {
  // if (!mainWindow.isVisible()) {
  showWindow();
  // }
}

function createWindow() {
  // create tray icon
  tray = new Tray(config.TRAY_ICON);
  // tray.on('right-click', toggleWindow);
  tray.on('right-click', () => {
    const contextMenu = Menu.buildFromTemplate([
      { label: 'Quit', click: () => { app.quit(); } },
    ]);
    tray.popUpContextMenu(contextMenu);
    // lastTrayIconBounds = bounds;
  });
  tray.setIgnoreDoubleClickEvents(true);
  tray.on('click', () => {
    toggleWindow();
  });
  tray.setToolTip('Bloom');

  // Create the browser window.
  mainWindow = new BrowserWindow({
    title: config.WINDOW_TITLE,
    width: config.WINDOW_DEFAULT_WIDTH,
    height: config.WINDOW_DEFAULT_HEIGHT,
    minWidth: config.WINDOW_MIN_WIDTH,
    minHeight: config.WINDOW_MIN_HEIGHT,
    webPreferences: {
      nodeIntegration: true,
    },
    icon: config.WINDOW_ICON,
  });

  // and load the index.html of the app.
  mainWindow.loadURL(config.WINDOW_URL);

  // Open the DevTools.
  // mainWindow.webContents.openDevTools()

  // Emitted when the window is closed.
  mainWindow.on('closed', () => {
    // Dereference the window object, usually you would store windows
    // in an array if your app supports multi windows, this is the time
    // when you should delete the corresponding element.
    mainWindow = null;
  });

  // globalShortcut.register('MediaPlayPause', () => {
  //   // Do stuff when Y and either Command/Control is pressed.
  //   console.log('PlayPause pressed');
  // });
}

app.on('ready', createWindow);


// Quit when all windows are closed.
app.on('window-all-closed', () => {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  // if (process.platform !== 'darwin') {
  //   app.quit();
  // }
  app.quit();
});

app.on('activate', () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (mainWindow === null) {
    createWindow();
  }
});

// const { Native } = require('./native/main');

// ipcMain.handle('message', async (event, someArgument) => {
//   console.log('IPCMain received: ', someArgument);
//   const res = await Native.call(someArgument);
//   console.log('IPCMain response received: ', res);
//   return res;
// });
