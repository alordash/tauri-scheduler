import { addNewTask, createNewTask } from './task.js';
import { setElementVisible, showTasksList } from './ui.js';
import { tryLoginUser } from './user.js';

const signOut = document.getElementById('signout');
const statusDiv = document.getElementById('status');
const taskDiv = document.getElementById('taskDiv');
const tasksDisplay = document.getElementById('tasksDisplay');

let userId = null;

async function signInOnClick() {
    const login = document.getElementById('login').value;
    const password = document.getElementById('password').value;

    const maybeUser = await tryLoginUser(login, password);

    if (maybeUser == null) {
        setElementVisible(statusDiv, true);
        setElementVisible(taskDiv, false);
        setElementVisible(tasksDisplay, false);
        setElementVisible(signOut, false);
        return;
    } else {
        setElementVisible(statusDiv, false);
        setElementVisible(taskDiv, true);
        setElementVisible(tasksDisplay, true);
        setElementVisible(signOut, true);
    }

    userId = maybeUser.id;

    await showTasksList(userId);
}

async function signOutClick() {
    setElementVisible(statusDiv, false);
    setElementVisible(taskDiv, false);
    setElementVisible(tasksDisplay, false);
    setElementVisible(signOut, false);

    userId = null;
    document.getElementById('login').value = '';
    document.getElementById('password').value = '';
}

async function createNewTaskOnClick() {
    const description = document.getElementById('taskDescription').value;
    const dueTime = document.getElementById('taskDueTime').value;

    const task = await createNewTask(description, false, dueTime, userId);
    const task_id = await addNewTask(task);

    await showTasksList(userId);
}

document.getElementById('signin').onclick = signInOnClick;
signOut.onclick = signOutClick;
document.getElementById('newTask').onclick = createNewTaskOnClick;