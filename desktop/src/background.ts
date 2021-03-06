import {
  app,
  protocol,
  BrowserWindow,
  ipcMain,
} from 'electron';
import axios from 'axios';
import { execFile, ChildProcess } from 'child_process';
import fs from 'fs';
import path from 'path';
import { autoUpdater } from 'electron-updater';
import { createProtocol } from './create_protocol';
import config from './config';


declare const __static: string; // eslint-disable-line

const CALL_URL = '/electronCall';
const UNIX_SOCKET_PATH = '/tmp/com.bloom42.bloom.sock';
const DAEMON_NAME = 'bloomd';

const isDevelopment = process.env.NODE_ENV !== 'production';


// Keep a global reference of the window object, if you don't, the window will
// be closed automatically when the JavaScript object is garbage collected.
let mainWindow: BrowserWindow | null = null;
// let tray: Tray | null = null;
// child is used to control the server
let child: ChildProcess | null = null;
const gotTheLock = app.requestSingleInstanceLock();

let appDataDir = app.getPath('appData');
// because sometimes, like with flatpak, UserConfigDir can already be app scoped
if (!appDataDir.includes(config.APP_ID)) {
  appDataDir = path.join(appDataDir, config.APP_ID);
}
fs.mkdirSync(appDataDir, { recursive: true });
app.setPath('userData', appDataDir);


// Scheme must be registered before the app is ready
protocol.registerSchemesAsPrivileged([
  { scheme: 'app', privileges: { secure: true, standard: true } },
]);

// show window
// function showWindow() {
//   // const position = getWindowPosition();
//   // mainWindow.setPosition(position.x, position.y, false);
//   mainWindow!.show();
//   mainWindow!.focus();
// }

// toggle window
// function toggleWindow() {
//   // if (!mainWindow.isVisible()) {
//   showWindow();
//   // }
// }

function killChild(): boolean {
  if (child !== null) {
    child.kill();
  }
  console.log('mainProcess: server stopped');
  return true;
}

function createWindow() {
  // create tray icon
  // tray = new Tray(config.TRAY_ICON);
  // tray.on('right-click', toggleWindow);
  // tray.on('right-click', (event, bounds) => {
  //   const contextMenu = Menu.buildFromTemplate([
  //     { label: 'Quit', click: () => { app.quit(); } },
  //   ]);
  //   tray!.popUpContextMenu(contextMenu);
  //   // lastTrayIconBounds = bounds;
  // });
  // tray.setIgnoreDoubleClickEvents(true);
  // tray.on('click', (event) => {
  //   toggleWindow();
  // });
  // tray.setToolTip('Bloom');
  let nodeIntegration = false;
  if (process.env.WEBPACK_DEV_SERVER_URL) {
    nodeIntegration = true;
  }

  // Create the browser window.
  mainWindow = new BrowserWindow({
    title: config.WINDOW_TITLE,
    width: config.WINDOW_DEFAULT_WIDTH,
    height: config.WINDOW_DEFAULT_HEIGHT,
    minWidth: config.WINDOW_MIN_WIDTH,
    minHeight: config.WINDOW_MIN_HEIGHT,
    webPreferences: {
      nodeIntegration,
      sandbox: !nodeIntegration,
      preload: path.join(__static, 'preload.js'),
    },
    icon: path.join(__static, config.WINDOW_ICON),
  });

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    // Load the url of the dev server if in development mode
    mainWindow.loadURL(process.env.WEBPACK_DEV_SERVER_URL as string);
    if (!process.env.IS_TEST) {
      mainWindow.webContents.openDevTools();
    }
  } else {
    createProtocol('app');
    // Load the index.html when not in development
    mainWindow.loadURL('app://./index.html');
    if (process.platform === 'darwin') {
      autoUpdater.checkForUpdatesAndNotify();
    }
  }

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}

// Quit when all windows are closed.
app.on('window-all-closed', () => {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  // if (process.platform !== 'darwin') {
  //   app.quit();
  // }
  killChild();
  app.quit();
});

app.on('activate', () => {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (mainWindow === null) {
    createWindow();
  }
});

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
// app.on('ready', createWindow);
if (!gotTheLock) {
  app.quit();
} else {
  app.on('second-instance', () => {
    // Someone tried to run a second instance, we should focus our window.
    if (mainWindow) {
      if (mainWindow.isMinimized()) {
        mainWindow.restore();
      }
      mainWindow.focus();
    }
  });

  // Create window
  app.on('ready', createWindow);
}

// Exit cleanly on request from parent process in development mode.
if (isDevelopment) {
  process.on('SIGTERM', () => {
    killChild();
    app.quit();
  });
}


ipcMain.on('server:start', () => {
  console.log('mainProcess: starting server');
  let bloomdPath = path.join(process.resourcesPath, DAEMON_NAME);
  if (process.env.WEBPACK_DEV_SERVER_URL) {
    bloomdPath = `./${DAEMON_NAME}`;
  }
  child = execFile(bloomdPath, (err, stdout, stderr) => {
    // print on exit
    if (stderr) {
      console.error(stderr.toString());
    }
    if (err) {
      console.error(err);
    }
  });
  if (child !== null && child.stdout !== null) {
    // stream stdout
    child.stdout!.setEncoding('utf8');
    child.stdout!.on('data', (data) => {
      console.log(data.toString());
    });
  }
  return true;
});

ipcMain.on('server:stop', killChild);

ipcMain.handle('core:call', async (event: any, message: any) => {
  const res = await axios({
    url: CALL_URL,
    method: 'post',
    data: JSON.stringify(message),
    socketPath: UNIX_SOCKET_PATH,
  });
  return res.data;
});
