from elinor import elinor


def evaluate(true_rels, pred_rels, metric):
    return elinor.raw_evaluate(true_rels, pred_rels, metric)
