const {
  app,
  BrowserWindow,
  Tray,
  Menu,
} = require('electron');
const path = require('path');

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
  tray = new Tray(path.join('public', 'tray_icon.png'));
  // tray.on('right-click', toggleWindow);
  tray.on('right-click', (event, bounds) => {
    const contextMenu = Menu.buildFromTemplate([
      { label: 'Quit', click: () => { app.quit(); } },
    ]);
    tray.popUpContextMenu(contextMenu);
    // lastTrayIconBounds = bounds;
  });
  tray.setIgnoreDoubleClickEvents(true);
  tray.on('click', (event) => {
    toggleWindow();
  });
  tray.setToolTip('Bloom');

  // Create the browser window.
  mainWindow = new BrowserWindow({
    title: 'Bloom',
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: true,
    },
  });

  // and load the index.html of the app.
  mainWindow.loadURL('http://localhost:8080');

  // Open the DevTools.
  // mainWindow.webContents.openDevTools()

  // Emitted when the window is closed.
  mainWindow.on('closed', () => {
    // Dereference the window object, usually you would store windows
    // in an array if your app supports multi windows, this is the time
    // when you should delete the corresponding element.
    mainWindow = null;
  });
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
