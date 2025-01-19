# **Rline Program Usage**

The `rline` program allows you to process files in a specified directory with options to exclude certain file extensions, ban specific directories, and enable recursive scanning.

---

## **Available Arguments**

| **Argument** | **Description**                                                                       | **Example**                                   |
| ------------ | ------------------------------------------------------------------------------------- | --------------------------------------------- |
| `-p`         | Specify the path to the target directory.                                             | `-p "C:\Users\user\project\a_coding_project"` |
| `-e`         | Ban specific file extensions. Provide a comma-separated list of extensions.           | `-e "exe, o, img"`                            |
| `-d`         | Ban specific directories. Provide a comma-separated list of directory names or paths. | `-d "assets, target"`                         |
| `-r`         | Enable recursive scanning through all subdirectories within the specified path.       | `-r`                                          |

---

## **Example Usage**

```bash
rline -d "assets\img" -e "o, exe" -p "C:\Users\user\project\a_coding_project" -r
```

---

## **Explanation**

1. **Path Specification (`-p`)**:
   Use the `-p` argument to provide the root directory where the program will begin its operation. This is mandatory.

2. **Ban File Extensions (`-e`)**:
   Use the `-e` argument to exclude files with specific extensions. Provide a comma-separated list without spaces.

   Example: `-e "exe, o, img"` will ignore files with `.exe`, `.o`, and `.img` extensions.

3. **Ban Directories (`-d`)**:
   Use the `-d` argument to exclude specific directories or subdirectories. Provide a comma-separated list of directory names or paths.

   Example: `-d "assets, target"` will ignore any directory named `assets` or `target`.

4. **Recursive Scan (`-r`)**:
   Add the `-r` flag to enable scanning through all subdirectories within the specified path recursively.

---

## **Downloading and Adding to PATH on Windows**

1. **Download the Release**:
   - Visit the [Releases](https://github.com/relextm19/Rline/releases) page of the GitHub repository.
   - Download the appropriate version of the `rline` program for Windows.

2. **Add to PATH**:
   - Move the downloaded file (e.g., `rline.exe`) to a folder of your choice (e.g., `C:\Programs\rline`).
   - Open the Start Menu, search for "Environment Variables," and select "Edit the system environment variables."
   - In the System Properties window, click "Environment Variables."
   - Under "System variables," find the `Path` variable and click "Edit."
   - Click "New" and add the path to the folder containing `rline.exe` (e.g., `C:\Programs\rline`).
   - Click "OK" to save the changes.
---


