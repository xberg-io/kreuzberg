Table 1: Current layout detection models in the LayoutParser model zoo

| Dataset         |       | Base Model'&#124; Large Model &#124; Notes   |                                                            |
|-----------------|-------|----------------------------------------------|------------------------------------------------------------|
| PubLayNet [38]  | F / M | M                                            | Layouts of modern acientific documents                     |
| PRImA (3]       | M     |                                              | Layouts of scanned modern magazines and scientific reports |
| Newspaper (17)  |       |                                              | Labut of and i caper for the both conten.                  |
| ableBank &#124; |       |                                              |                                                            |
| Dataset [       | F / M |                                              | ayouts of history Japanese document                        |

1 orga dion car, or ta mad and as ice Were a i Reait 0 or Realit 101. 2o0 in coming months.

layout data structures, which are optimized for efficiency and versatility. 3) When necessary, users can employ existing or customized OCR models via the unified API provided in the OCR module. 4) LayoutParser comes with a set of utility functions for the visualization and storage of the layout data. 5) LayoutParser is also highly customizable, via its integration with functions for layout data annotation and model training. We now provide detailed descriptions for each

## 3.1 Layout Detection Models

In LayoutParser, a layout model takes a document image as an input and generates a list of rectangular boxes for the target content regions. Different from traditional methods, it relies on deep convolutional neural networks rather than manually curated rules to identify content regions. It is formulated as an object detection problem and state-of-the-art models like Faster R-CNN 28] and Mask R-CNN 12] are used. This yields prediction results of high accuracy and makes it possible to build a concise, generalized interface for layout detection. LayoutParser, built upon Detectron2 35], provides a minimal API that can perform layout detection with only four lines of code in Python:

```text
import layoutparser as lp 2 image = cv2. imread("image_file") # load images 3 model = 1p. Detectron2LayoutModel ( "1p://PubLayllet/faster_rcnn_R_50_FPN_3x/config") · layout = model. detect (image)
```

LayoutParser provides a wealth of pre-trained model weights using various datasets covering different languages, time periods, and document types. Due to domain shift [7], the prediction performance can notably drop when models are applied to target samples that are significantly different from the training dataset. As document structures and layouts vary greatly in different domains, it is important to select models trained on a dataset similar to the test samples. A semantic syntax is used for initializing the model weights in LayoutParser, using both the dataset name and model name 1p://&lt;dataset-name&gt;/&lt;model-architecture-name&gt;.
