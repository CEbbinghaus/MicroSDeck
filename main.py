import pathlib
import subprocess
import asyncio
import os

HOME_DIR = str(pathlib.Path(os.getcwd()).parent.parent.resolve())
PARENT_DIR = str(pathlib.Path(__file__).parent.resolve())

class Plugin:
    backend_proc = None
    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded
    async def _main(self):
        # startup
        while True:
            self.return_code = None
            self.backend_proc = subprocess.Popen([PARENT_DIR + "/bin/backend"])
            
            while True:
                if(self.backend_proc.returncode != None):
                    self.return_code = self.backend_proc.returncode
                    break
                await asyncio.sleep(1)

            match self.return_code:
                # SIGKILL, SIGTERM, The Process was killed by the user
                case 137 | 143:
                    break
                
                # SIGABRT The program is assumed to have crashed
                case 134:
                    print("SIGABRT was returned")
                    
                # The Program exited normally? This happens when its cancelled with Ctrl+C or
                # one of the threads exited without an error. Restart for good measure
                case 0:
                    print("Program exited successfully")
                
                case _code:
                    print("Program exited with exit code: " + _code)
                    break;
         # Function used to clean up a plugin when it's told to unload by Decky-Loader
    
    async def _unload(self):
        self.backend_proc.kill();