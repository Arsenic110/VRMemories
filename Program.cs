using System.Diagnostics;
using System.Reflection;
using System.Globalization;

namespace VRMemories
{
    class Program
    {
        static int cooldown = 300000; //5 minutes

        static void Main(string[] args)
        {
            UnpackAssembly();

            //TestOpenVRApi();

            ScreenshotLoop();
        }

        private static void ScreenshotLoop()
        {
            VRApp vrapp = new VRApp();
            long target = DateTimeOffset.Now.ToUnixTimeMilliseconds();

            while (true) //mental anguish.
            {
                if (DateTimeOffset.Now.ToUnixTimeMilliseconds() >= target)
                {
                    vrapp.Capture();
                    target = DateTimeOffset.Now.ToUnixTimeMilliseconds() + cooldown;
                }
            }
        }

        private static void TestOpenVRApi()
        {
            Valve.VR.OpenVR.Chaperone.AreBoundsVisible();
        }

        private static void PrintAssemblyNames()
        {
            var resourceNames = Assembly.GetExecutingAssembly().GetManifestResourceNames();
            foreach (var resourceName in resourceNames)
            {
                Console.WriteLine(resourceName);
            }
        }

        private static void UnpackAssembly()
        {
            string fileName = "openvr_api.dll";
            if (File.Exists(fileName))
                return;

            var executingAssembly = Assembly.GetExecutingAssembly();

            var path = "VRmemories." + fileName;

            using (Stream stream = executingAssembly.GetManifestResourceStream("VRMemories.openvr_api.dll"))
            {
                if (stream == null)
                    return;

                var assemblyRawBytes = new byte[stream.Length];
                stream.Read(assemblyRawBytes, 0, assemblyRawBytes.Length);

                using (FileStream fs = new FileStream("openvr_api.dll", FileMode.CreateNew))
                {
                    fs.Write(assemblyRawBytes);
                }
            }


        }
    }
}