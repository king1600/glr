from time import time
from shutil import rmtree
from platform import system
from sys import argv, exit as sys_exit
from os.path import join, exists, splitext, dirname
from os import stat, walk, utime, mkdir, getcwd, system as launch

def local(path):
    if CWD in path:
        return path.split(CWD)[1][1:]
    return path

def get_config():
    default = {
        'ext': '.c',
        'head': '.h',
        'cc': 'clang',
        'src': local('src'),
        'build': local('build'),
        'debug': {
            'cc': '-g -Wall -O0',
            'ld': '',
        },
        'release': {
            'cc': '-g -Wall -Ofast -march=native',
            'ld': '-O3',
        }
    }
    return {
        'Linux': dict(default, **{
            'bin': 'glr',
            'libs': ['-pthread']
        }),
        'Windows': dict(default, **{
            'bin': 'glr.exe',
            'libs': ['-lws2_32']
        }),
    }[system()]

CWD = getcwd()
CFG = get_config()
CLEAN = 'clean' in argv
RELEASE = 'release' in argv

def clean():
    rmtree(CFG['build'])
    mkdir(CFG['build'])

def should_recompile(source, obj, dest):
    src_stat = stat(source)
    if stat(obj).st_mtime < src_stat.st_mtime:
        return True
    with open(dest + '.d', 'r') as dependencies:
        for line in dependencies:
            line = line.strip()
            if line and line.endswith(CFG['head'] + ':'):
                header = stat(line[:-1])
                if header.st_mtime > src_stat.st_mtime:
                    utime(source, (src_stat.st_atime, time()))
                    return True

def build():
    objects = []
    recompiled = False
    flags = CFG['release' if RELEASE else 'debug']
    sources = (
        local(join(dirname, filename))
        for dirname, subdirs, files in walk(CFG['src'])
        for filename in files if filename.endswith(CFG['ext'])
    )

    for source in sources:
        dest = splitext(source.replace(CFG['src'], CFG['build']))[0]
        obj = dest + '.o'
        objects.append(obj)
        if not exists(obj) or should_recompile(source, obj, dest):
            recompiled = True
            obj_dir = dirname(obj)
            if not exists(obj_dir):
                mkdir(obj_dir)
            command = '{} {} -MMD -MP -c {} -o {}'.format(
                CFG['cc'], flags['cc'], source, obj)
            print(command)
            if launch(command) != 0:
                sys_exit(-1)
        
    if recompiled:
        command = '{} {} {} -o {} {}'.format(
            CFG['cc'], flags['ld'], ' '.join(objects), CFG['bin'], ' '.join(CFG['libs']))
        print(command)
        sys_exit(launch(command))

(clean if CLEAN else build)()
