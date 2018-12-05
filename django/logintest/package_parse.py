# from androguard.misc import AnalyzeAPK
# #a,d,dx = AnalyzeAPK('app-debug.apk')
# a, d, dx = AnalyzeAPK('uu.apk')
# print(dir(dx))
# # print(dir(d))
# # print(dir(dx))
# print(a.get_androidversion_name())
# print(a.is_valid_APK())
# print(a.get_app_name())
# print(a.get_app_icon())

# print(a.get_files_information())
# print(a.is_signed())

# import os
# s = os.path.getsize('uu.apk')
# print(round(s/1024/1024, 1))

import zipfile
import re
# with zipfile.ZipFile('uu.apk') as apkzip:
#     print('testzip:', apkzip.testzip())
#     for item in apkzip.namelist():
#         result = re.search('res/mipmap-xxxhdpi-.*/ic_launcher.png', item)
#         if result:
#             print(result.group(0))
#             with apkzip.open(result.group(0)) as icon:
#                 with open('icon2.png', 'wb') as icon2:
#                     i = icon.read()
#                     icon2.write(i)
#         if item == 'AndroidManifest.xml':
#             print(item)
#             with apkzip.open(item, 'r') as manifest:
#                 with open('manifest.xml', 'wb') as mani:
#                     m = manifest.read()
#                     mani.write(m)
import fnmatch
import plistlib
with zipfile.ZipFile('EIntegrate.ipa') as ipazip:
    files = ipazip.namelist()
    for i in files:
        print(i)
    info_plist = fnmatch.filter(files, 'Payload/*.app/Info.plist')[0]
    info_plist_bin = ipazip.read(info_plist)
    plist_root = plistlib.loads(info_plist_bin)
    print(plist_root)
    print(plist_root['CFBundleShortVersionString'])
    print(plist_root['CFBundleIcons']['CFBundlePrimaryIcon']['CFBundleIconFiles'][-1])
    print(plist_root['CFBundleIdentifier'])
    print(plist_root['CFBundleDisplayName'])
    icon_file_name = plist_root['CFBundleIcons']['CFBundlePrimaryIcon']['CFBundleIconFiles'][-1]
    icon_files = fnmatch.filter(files, '*%s*.png' % icon_file_name)
    print(icon_files[-1])
    with ipazip.open(icon_files[-1]) as icon:
        with open('appicon.png', 'wb') as saveicon:
            i = icon.read()
            saveicon.write(i)
    with open('info.plist', 'wb') as f:
        plistlib.dump(plist_root, f)