try:
    import Image
except ImportError:
    from PIL import Image
import pytesseract

pytesseract.pytesseract.tesseract_cmd = 'tesseract'
print(pytesseract.image_to_string(Image.open('/Users/zzq9171/Downloads/ValidationCode.jpg')))