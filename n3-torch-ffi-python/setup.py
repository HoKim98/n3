from setuptools import find_packages, setup

author = {
    'name': 'Ho Kim',
    'email': 'ho.kim@gnu.ac.kr',
}


def read(filename):
    with open(filename, 'r', encoding='utf-8') as f:
        return f.read()


def read_requirements():
    rs = read('requirements.txt').split('\n')
    rs = [r.strip() for r in rs]
    rs = [r for r in rs if len(r) > 0 and not r.startswith('#')]
    return rs


setup(
    version=read('VERSION'),

    name='n3',
    description='Nerual Network Notation',
    long_description=read('README.md'),
    long_description_content_type='text/markdown',
    url=r'https://github.com/kerryeon/n3',

    author=author['name'],
    author_email=author['email'],
    maintainer=author['name'],
    maintainer_email=author['email'],
    license='MIT',

    install_requires=read_requirements(),
    packages=find_packages(),

    entry_points={
        'console_scripts': [
            'n3=n3.exec:route',
        ],
    },

    include_package_data=True,
    zip_safe=False,
)
