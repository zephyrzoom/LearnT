import logging
import sys
# logging.basicConfig(filename='today.log', level=logging.DEBUG)
# log = logging.getLogger('log test')
# log.info('abc')

root = logging.getLogger('boc-ml')
root.setLevel(logging.DEBUG)

ch = logging.FileHandler('log/today.log')
ch.setLevel(logging.DEBUG)
formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
ch.setFormatter(formatter)
root.addHandler(ch)

root.info('asdfsdf')