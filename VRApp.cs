using OVRSharp.Exceptions;
using OVRSharp.Graphics.DirectX;
using Valve.VR;
using System;
using System.Drawing;
using System.Diagnostics;

namespace VRMemories
{
    public class VRApp : OVRSharp.Application
    {
        DirectXCompositor compositor;
        string path;

        public VRApp() : base(ApplicationType.Background)
        {
            this.compositor = new DirectXCompositor();

            //default
            path = Path.Join(Environment.GetFolderPath(Environment.SpecialFolder.MyPictures), "VRMemories");
            if(!Directory.Exists(path))
            {
                Directory.CreateDirectory(path);
            }
        }

        public void Capture()
        {
            Bitmap bt = compositor.GetMirrorImage();
            string fp = Path.Join(path, GetUniqueFilename());

            Console.Write($"Capturing {fp} ... ");
            Debug.Write($"Capturing {fp} ... ");

            using (FileStream fs = new FileStream(fp, FileMode.OpenOrCreate))
            {
                bt.Save(fs, System.Drawing.Imaging.ImageFormat.Png);
                bt.Dispose();
            }

            Console.Write("complete.\n");
            Debug.Write("complete.\n");
        }

        public string GetUniqueFilename()
        {
            DateTime d = DateTime.Now;

            //YYYY-MM-DD_HH-mm-ss-SSS
            return $"{d.Year}-{(d.Month + "").PadLeft(2, '0')}-{(d.Day + "").PadLeft(2, '0')}_{(d.Hour + "").PadLeft(2, '0')}-{(d.Minute + "").PadLeft(2, '0')}-{(d.Second + "").PadLeft(2, '0')}-{(d.Millisecond + "").PadLeft(3, '0')}.png";
        }
    }
}
