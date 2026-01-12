using System;
using System.Design;
using Microsoft.VisualStudio.Shell;

namespace RiwaqArch
{
    internal static class StopServerCommand
    {
        public static async System.Threading.Tasks.Task InitializeAsync(RiwaqArchPackage package)
        {
            await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync();

            var commandId = new CommandID(GuidList.guidRiwaqArchCmdSet, (int)PkgCmdIDList.cmdidStopServer);
            var menuItem = new OleMenuCommand((sender, e) => ExecuteAsync(package).Wait(), commandId);
            menuItem.BeforeQueryStatus += (sender, e) => MenuItem_BeforeQueryStatus(sender, e);

            var mcs = await package.GetServiceAsync(typeof(IMenuCommandService)) as OleMenuCommandService;
            mcs?.AddCommand(menuItem);
        }

        private static void MenuItem_BeforeQueryStatus(object sender, EventArgs e)
        {
            var button = (OleMenuCommand)sender;
            button.Enabled = RiwaqArchPackage.ServerManager.IsRunning;
        }

        private static async System.Threading.Tasks.Task ExecuteAsync(RiwaqArchPackage package)
        {
            await ThreadHelper.JoinableTaskFactory.SwitchToMainThreadAsync();

            await RiwaqArchPackage.ServerManager.StopServerAsync();
        }
    }
}
