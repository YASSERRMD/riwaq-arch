using System;
using System.Design;
using Microsoft.VisualStudio.Shell;

namespace RiwaqArch
{
    internal static class StartServerCommand
    {
        public static async System.Threading.Tasks.Task InitializeAsync(RiwaqArchPackage package)
        {
            await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync();

            var commandId = new CommandID(GuidList.guidRiwaqArchCmdSet, (int)PkgCmdIDList.cmdidStartServer);
            var menuItem = new OleMenuCommand((sender, e) => ExecuteAsync(package).Wait(), commandId);
            menuItem.BeforeQueryStatus += (sender, e) => MenuItem_BeforeQueryStatus(sender, e);

            var mcs = await package.GetServiceAsync(typeof(IMenuCommandService)) as OleMenuCommandService;
            mcs?.AddCommand(menuItem);
        }

        private static void MenuItem_BeforeQueryStatus(object sender, EventArgs e)
        {
            var button = (OleMenuCommand)sender;
            button.Enabled = !RiwaqArchPackage.ServerManager.IsRunning;
        }

        private static async System.Threading.Tasks.Task ExecuteAsync(RiwaqArchPackage package)
        {
            await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync();

            var success = await RiwaqArchPackage.ServerManager.StartServerAsync();

            if (!success)
            {
                VsShellUtilities.ShowMessageBox(
                    package,
                    "Failed to start Riwaq server. Check settings and try again.",
                    "Server Start Failed",
                    OLEMSGICON.OLEMSGICON_CRITICAL,
                    OLEMSGBUTTON.OLEMSGBUTTON_OK,
                    OLEMSGDEFBUTTON.OLEMSGDEFBUTTON_FIRST);
            }
        }
    }
}
