// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';
var today = new Date();
// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
function makeid(length: number) {
	var result           = '';
	var characters       = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
	var charactersLength = characters.length;
	for ( var i = 0; i < length; i++ ) {
	   result += characters.charAt(Math.floor(Math.random() * charactersLength));
	}
	return result;
 }

 
export function activate(context: vscode.ExtensionContext) {

	console.log('Congratulations, your extension "vs-code-rust-new-verion" is now active!');

	let today = new Date().toLocaleDateString()

    console.log(today)

	let disposable = vscode.commands.registerCommand('vs-code-rust-new-verion.helloWorld', () => {
		// The code you place here will be executed every time your command is executed
		const { activeTextEditor } = vscode.window;

        if (activeTextEditor) {
            const { document } = activeTextEditor;
            if (document) {
				let template=`pub fn `+document.filename+"_get_verison"+` -> &'static str {
				return `+document.fileName+`   verison is `+today+` ID is: `+makeid(8) + `
			} \n`
				
			
    // check if there is no selection
    if (activeTextEditor.selection.isEmpty) {
      // the Position object gives you the line and character where the cursor is
      const position = activeTextEditor.selection.active;
    
                const textEdits: vscode.TextEdit[] = [];
                textEdits.push(vscode.TextEdit.insert(new vscode.Position(position.line,position.character),template));

                const workEdits = new vscode.WorkspaceEdit();
                workEdits.set(document.uri, textEdits); // give the edits
                vscode.workspace.applyEdit(workEdits); // apply the edits
			}
		}
        }
		// Display a message box to the user
		vscode.window.showInformationMessage('Verison number is added to code ');
	});

	context.subscriptions.push(disposable);
}




// this method is called when your extension is deactivated
export function deactivate() {}
